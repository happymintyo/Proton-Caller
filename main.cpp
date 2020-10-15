#include <iostream>
#include <string>
#include <cstring>
#include <sys/stat.h>
#include <filesystem>

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

        void proton_call() const {
            string exec_ = proton_path + proton + "/proton run " + program;
            char cmd[200];
            strcpy(cmd, exec_.c_str());
            system(cmd);
        }

        void setup() {
            string _home = getenv("HOME");

            if (custom) {
                program = _argv3;
                proton_path = _argv2;
                return;
            } else {
                proton = _argv1;
                program = _argv2;
                if (_argv1 == "5"){proton = "5.0";}
                proton_path = _home + "/.steam/steam/steamapps/common/Proton\\ ";
                return;
            }
        }

};


void help() {
    cout << "Usage: [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n 'proton-call 5 ./foo.exe'\n 'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n";
}

/*
void checker(ProtonClass &pc) {
    string _home = getenv("HOME");

    if (pc.custom) {
        pc.program = pc._argv3;
        pc.proton_path = pc._argv2;
        return;
    } else {
        pc.proton = pc._argv1;
        pc.program = pc._argv2;
        if (pc._argv1 == "5"){pc.proton = "5.0";}
        pc.proton_path = _home + "/.steam/steam/steamapps/common/Proton\\ ";
        return;
    }
}
*/

int main(int argc, char *argv[]) {
    ProtonClass ProtonObject;
    ProtonObject.steam = "STEAM_COMPAT_DATA_PATH";
    string version = "1.0.3";
    cout << "Proton Caller by Avery Murray version: " << version << "\n\n";

    // check for compat data path
    if (getenv(ProtonObject.steam) == nullptr) {
        cout << "Please add " << ProtonObject.steam << " to your environment:\n\n export" << ProtonObject.steam << "=$HOME/proton/\n";
        help();exit(EXIT_FAILURE);
    }

    cout << ProtonObject.steam << " located at: " << getenv(ProtonObject.steam) << "\n\n";
    if (argc == 1){help();exit(EXIT_FAILURE);}
    if (argv[1] != nullptr){ProtonObject._argv1 = argv[1];}else{exit(EXIT_FAILURE);}
    if (argv[2] != nullptr){ProtonObject._argv2 = argv[2];}else{exit(EXIT_FAILURE);}

    if (ProtonObject._argv1 == "-h"){help();exit(EXIT_SUCCESS);}
    if (ProtonObject._argv1 == "-c") {
        ProtonObject.custom = true;
        if (argv[3] != nullptr){
            ProtonObject._argv3 = argv[3];
        } else{exit(EXIT_FAILURE);}
    } else{ProtonObject.custom = false;}


    ProtonObject.setup();
    ProtonObject.checkfs();
    ProtonObject.proton_call();

    return 0;
}
