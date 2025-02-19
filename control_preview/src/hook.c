#include "control_preview.h"
#include <string.h>
#include <uxtheme.h>
#include <wingdi.h>
#include <winnt.h>
#include "../minhook/include/Minhook.h"

// Utility function: Get OS build number.
typedef NTSTATUS (NTAPI *RtlGetVersion_t)(PRTL_OSVERSIONINFOEXW);
DWORD GetOSBuild()
{
    static RTL_OSVERSIONINFOEXW osvi = { 0 };
    
    if (osvi.dwOSVersionInfoSize == 0)
    {
        HMODULE hModNtdll = GetModuleHandleW(L"ntdll.dll");
        RtlGetVersion_t pfnRtlGetVersion = (RtlGetVersion_t)GetProcAddress(hModNtdll, "RtlGetVersion");
        if (pfnRtlGetVersion)
        {
            pfnRtlGetVersion(&osvi);
        }
    }
    
    return osvi.dwBuildNumber;
}

typedef struct _UXTHEMEFILE
{
	char header[7]; // must be "thmfile"
	LPVOID sharableSectionView;
	HANDLE hSharableSection;
	LPVOID nsSectionView;
	HANDLE hNsSection;
	char end[3]; // must be "end"

} UXTHEMEFILE;

typedef HRESULT(WINAPI *GetThemeDefaults_t)(
	LPCWSTR pszThemeFileName,
	LPWSTR  pszColorName,
	DWORD   dwColorNameLen,
	LPWSTR  pszSizeName,
	DWORD   dwSizeNameLen
);
GetThemeDefaults_t GetThemeDefaults = NULL;

typedef HRESULT(WINAPI *LoaderLoadTheme_t)(
	HANDLE      hThemeFile,
	HINSTANCE   hThemeLibrary,
	LPCWSTR     pszThemeFileName,
	LPCWSTR     pszColorParam,
	LPCWSTR     pszSizeParam,
	OUT HANDLE *hSharableSection,
	LPWSTR      pszSharableSectionName,
	int         cchSharableSectionName,
	OUT HANDLE *hNonsharableSection,
	LPWSTR      pszNonsharableSectionName,
	int         cchNonsharableSectionName,
	PVOID       pfnCustomLoadHandler,
	OUT HANDLE *hReuseSection,
	int         a,
	int         b,
	BOOL        fEmulateGlobal
);
typedef HRESULT(WINAPI *LoaderLoadTheme_t_win11)(
	HANDLE      hThemeFile,
	HINSTANCE   hThemeLibrary,
	LPCWSTR     pszThemeFileName,
	LPCWSTR     pszColorParam,
	LPCWSTR     pszSizeParam,
	OUT HANDLE *hSharableSection,
	LPWSTR      pszSharableSectionName,
	int         cchSharableSectionName,
	OUT HANDLE *hNonsharableSection,
	LPWSTR      pszNonsharableSectionName,
	int         cchNonsharableSectionName,
	PVOID       pfnCustomLoadHandler,
	OUT HANDLE *hReuseSection,
	int         a,
	int         b
);
LoaderLoadTheme_t LoaderLoadTheme = NULL;


typedef HTHEME(WINAPI *OpenThemeDataFromFile_t)(
	UXTHEMEFILE *lpThemeFile,
	HWND         hWnd,
	LPCWSTR      pszClassList,
	DWORD        dwFlags
);
OpenThemeDataFromFile_t OpenThemeDataFromFile = NULL;

UXTHEMEFILE *g_pGlobalTheme = NULL;

void FreeTheme(UXTHEMEFILE *pFile)
{
	if (pFile)
	{
		if (pFile->sharableSectionView)
		{
			UnmapViewOfFile(pFile->sharableSectionView);
		}
		if (pFile->nsSectionView)
		{
			UnmapViewOfFile(pFile->nsSectionView);
		}

		CloseHandle(pFile->hNsSection);
		CloseHandle(pFile->hSharableSection);

		free(pFile);
	}
}

DWORD WINAPI DelayFreeThread(void *lParam)
{
	Sleep(1000);
	FreeTheme((UXTHEMEFILE *)lParam);
	return 0;
}

HRESULT LoadThemeFile(LPCWSTR lpszPath)
{
    HRESULT hr = S_OK;

    if (g_pGlobalTheme)
    {
        FreeTheme(g_pGlobalTheme);
        g_pGlobalTheme = NULL;
    }

    g_pGlobalTheme = malloc(sizeof(UXTHEMEFILE));
    ZeroMemory(g_pGlobalTheme, sizeof(UXTHEMEFILE));

    WCHAR szColor[MAX_PATH];
    WCHAR szSize[MAX_PATH];

    hr = GetThemeDefaults(
        lpszPath,
        szColor,
        ARRAYSIZE(szColor),
        szSize,
        ARRAYSIZE(szSize)
    );
    if (FAILED(hr))
    {
        FreeTheme(g_pGlobalTheme);
        g_pGlobalTheme = NULL;
        return hr;
    }

    HANDLE hSharable, hNonSharable;
    if (GetOSBuild() < 20000
        ? LoaderLoadTheme(0LL, 0LL, lpszPath, szColor, szSize, &hSharable, 0LL, 0, &hNonSharable, 0LL, 0, 0LL, 0LL, 0, 0, 0)
        : ((LoaderLoadTheme_t_win11)LoaderLoadTheme)(
            0LL,
            0LL,
            lpszPath,
            szColor,
            szSize,
            &hSharable,
            0LL,
            0,
            &hNonSharable,
            0LL,
            0,
            0LL,
            0LL,
            0,
            0))
    {
        FreeTheme(g_pGlobalTheme);
        g_pGlobalTheme = NULL;
        return hr;
    }

    memcpy(g_pGlobalTheme->header, "thmfile", 7);
    memcpy(g_pGlobalTheme->end, "end", 3);
    g_pGlobalTheme->sharableSectionView = MapViewOfFile(hSharable, 4, 0, 0, 0);
    g_pGlobalTheme->hSharableSection = hSharable;
    g_pGlobalTheme->nsSectionView = MapViewOfFile(hNonSharable, 4, 0, 0, 0);
    g_pGlobalTheme->hNsSection = hNonSharable;

    return S_OK;
}

HRESULT LoadGlobalTheme(LPCWSTR lpszThemeFilePath)
{   
    HMODULE hUxTheme = GetModuleHandleW(L"uxtheme.dll");
    
    if (!hUxTheme)
    {
        return E_FAIL;
    }
    
    // Undocumented exports from uxtheme:
	GetThemeDefaults = (GetThemeDefaults_t)GetProcAddress(hUxTheme, (LPCSTR)7);
	LoaderLoadTheme = (LoaderLoadTheme_t)GetProcAddress(hUxTheme, (LPCSTR)92);
	OpenThemeDataFromFile = (OpenThemeDataFromFile_t)GetProcAddress(hUxTheme, (LPCSTR)16);

	HRESULT hr = LoadThemeFile(lpszThemeFilePath);
    
	if (FAILED(hr))
    {
		return E_FAIL;
    }
    
    return S_OK;
}

// Windows API hooks:

HTHEME __stdcall OpenThemeData_Hook(HWND hwnd, LPCWSTR pszClassList)
{
	return OpenThemeDataFromFile(g_pGlobalTheme, hwnd, pszClassList, 0);
}

INITRESULT InitializeHooks()
{
    if (MH_Initialize() != MH_OK)
    {
        return CPIR_FAILED_MINHOOK_INIT;
    }
    
    void *pfnOpenThemeDataOrig = NULL;
    if (MH_CreateHook(&OpenThemeData, &OpenThemeData_Hook, pfnOpenThemeDataOrig))
    {
        return CPIR_FAILED_MINHOOK_HOOK;
    }
    
    return CPIR_SUCCEEDED;
}

INITRESULT Uninitialize()
{
    if (MH_Uninitialize() != MH_OK)
    {
        return CPIR_FAILED;
    }
    
    return CPIR_SUCCEEDED;
}