/*
DO NOT USE THIS IT IS NOT FINISHED

*/



#include <iostream>
#include <string>
#include <sys/stat.h>
#ifdef __cplusplus
# include <lua5.2/lua.hpp>
#else
# include <lua5.2/lua.h>
# include <lua5.2/lualib.h>
# include <lua5.2/lauxlib.h>
#endif
using namespace std;


class ProtonClass {
    public:
        string _argv1;
        string _argv2;
        string _argv3;
        const char *steam;
        string proton;
        string program;
        string proton_path;
        bool custom;

        void proton_call() {
            // cout << proton_path;
            const char *cmd;
            cmd = "env";
            system(cmd);
        }
};

void help() {
    cout << "Usage: [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n 'proton-call 5 ./foo.exe'\n 'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n";
}

void checker(ProtonClass &pc) {
    if (pc._argv1 == "-h") {
        help();
        exit (0);
    }
    if (pc._argv1 == "-c") {
        pc.custom = true;
        pc.program = pc._argv3;
        pc.proton_path = pc._argv2;
        return;
    } else {
        pc.custom = false;
        if (pc._argv1 == "5") {
            pc.proton = "5.0";
        } else {
            pc.proton = pc._argv1;
            pc.program = pc._argv2;
        }
        pc.proton_path = "$HOME/.steam/steam/steamapps/common/Proton " + pc.proton + "/proton run ";
        return;
    }

}


int main(int argc, char * argv[]) {
    ProtonClass pc;
    string version = "1.0.0a";
    pc.steam = "STEAM_COMPAT_DATA_PATH";

    cout << "Proton Caller by Avery Murray version: " << version << "\n\n";

    if (getenv(pc.steam) == NULL) {
        const char *cmd1 = "mkdir $HOME/proton";
        const char *cmd2 = "export STEAM_COMPAT_DATA_PATH=$HOME/proton";
        system(cmd1);
        system(cmd2);
    }

    cout << pc.steam << " located at: " << getenv(pc.steam) << endl;

    if (argv[1] != NULL) {
        pc._argv1 = argv[1];
    } else {
        cout << "Missing arguments.\n";
        return 255;
    }
    if (argv[2] != NULL) {
        pc._argv2 = argv[2];
    } else {
        cout << "missing arguments\n";
        return 255;
    }
    if (argv[3] != NULL) {
        pc._argv3 = argv[3];
    }

    // check for arguments
    checker(pc);

    if (pc.custom) {
        pc.program = argv[3];
    }

    pc.proton_call();

    return 0;

}