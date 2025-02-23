#include "../src/restyle.h"

HINSTANCE g_hInstance;

BOOL WINAPI DllMain(HINSTANCE hInstance, DWORD dwReason, LPVOID lpvReserved)
{
    switch (dwReason)
    {
        case DLL_PROCESS_ATTACH:
        {
            g_hInstance = hInstance;
            break;
        }
        
        case DLL_THREAD_ATTACH:
        {
            break;
        }
        
        case DLL_THREAD_DETACH:
        {
            break;
        }
        
        case DLL_PROCESS_DETACH:
        {
            break;
        }
    }
    
    return TRUE;
}