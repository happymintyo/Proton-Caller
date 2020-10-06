#include <iostream>
#include <string>
#include <stdlib.h>
#include <cstring>
using namespace std;

// int system(const char* command);


/*  This program currently DOES NOT work. please use the normal python script until this is outside of the src folder.
    I am rewriting it in C++ to help with speed, get more features, have less overhead and resources. */


class env {
    public:
        string version;
        string _argv1;
        string _argv2;
        string steam;
        string proton;
        string program;
        string proton_path;
        bool custom;

        int execl(const char *path, const char *args);

        /* int proton_call() {
            EIUFWHRUFHLDUFHURWLEHFKRHFLKRJHFR
            GWRRLGKHWRUHJRWLSGRK;GSJR
            GJSR'LGJSRLKGSRD
            G;LSR
            GL'SRDG;LS'
            RD;GLS;F'GL'S;FGL
            SFG
            S
            GS
            FGS
            'FG
            'SFDG'
            S


            return 0;
        } */
};


void help() {
    cout << "Usage: [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n 'proton-call 5 ./foo.exe'\n 'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n";
    return;
}


int main(int argc, char *argv[]) {
    int exit_val;
    
    env pc;
    pc.version = "1.0.0a";
    pc.steam = "STEAM_COMPAT_DATA_PATH";
    cout << "Proton Caller by Avery Murray version: " << pc.version << endl;

    
    if (argv[1] != NULL) {
        pc._argv1 = argv[1];
    } else {
        cout << "Arguments missing.\n";
    }
    if (argv[2] != NULL) {
        pc._argv2 = argv[2];
    }

    if (pc._argv1 == "-h") {
        help();
        return 0;
    }
    if (pc._argv1 == "-c") {
        pc.custom = true;
        pc.program = argv[3];
        pc.proton_call();
        return 0;
    } else {
        pc.custom = false;
        if (pc._argv1 == "5") {
            pc.proton = "5.0";
        } else {
            pc.proton = argv[1];
            pc.program = argv[2];
        }
        
    }

    return 0;
}
