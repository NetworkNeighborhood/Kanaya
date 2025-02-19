#include <windows.h>

typedef enum _INITRESULT
{
    CPIR_SUCCEEDED,
    CPIR_FAILED,
    CPIR_FAILED_MINHOOK_INIT,
    CPIR_FAILED_MINHOOK_HOOK,
} INITRESULT;

INITRESULT InitializeHooks();
INITRESULT Uninitialize();
HRESULT LoadGlobalTheme(LPCWSTR lpszThemeFilePath);