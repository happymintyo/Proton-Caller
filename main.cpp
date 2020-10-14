#include <iostream>
#include <string>
#include <cstring>
using namespace std;


class ProtonClass {
    public:
        string _argv1;
        string _argv2;
        string _argv3;
        const char *steam{};
        string proton;
        string program;
        string proton_path;
        bool custom{};


        void proton_call() {
            string exec_ = proton_path + proton + "/proton run " + program;
            char cmd[200];
            strcpy(cmd, exec_.c_str());
            system(cmd);
        }
};


void help() {
    cout << "Usage: [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n 'proton-call 5 ./foo.exe'\n 'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n";
}


void checker(ProtonClass &pc) {
    string _home = getenv("HOME");

    if (pc._argv1 == "-c") {
        pc.custom = true;
        pc.program = pc._argv3;
        pc.proton_path = pc._argv2;
        return;
    } else {
        pc.custom = false;
        pc.proton = pc._argv1;
        pc.program = pc._argv2;
        if (pc._argv1 == "5"){pc.proton = "5.0";}
        pc.proton_path = _home + "/.steam/steam/steamapps/common/Proton\\ ";
        return;
    }
}


int main(int argc, char *argv[]) {
    ProtonClass pc;
    pc.steam = "STEAM_COMPAT_DATA_PATH";
    string version = "1.1.0";
    cout << "Proton Caller by Avery Murray version: " << version << "\n\n";

    // check for compat data path
    if (getenv(pc.steam) == nullptr) {
        cout << "Please add " << pc.steam << " to your environment:\n\n export" << pc.steam << "=$HOME/proton/\n";
        help();exit(EXIT_FAILURE);
    }

    cout << pc.steam << " located at: " << getenv(pc.steam) << "\n\n";
    if (argc == 1){help();exit(EXIT_FAILURE);}
    if (argv[1] != nullptr){pc._argv1 = argv[1];}else{exit(EXIT_FAILURE);}
    if (argv[2] != nullptr){pc._argv2 = argv[2];}else{exit(EXIT_FAILURE);}
    if (argv[3] != nullptr){pc._argv3 = argv[3];}else{exit(EXIT_FAILURE);}

    checker(pc);
    pc.proton_call();

    return 0;
}
