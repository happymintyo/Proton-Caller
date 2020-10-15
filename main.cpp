#include <iostream>
#include <string>
#include "src/ProtonCaller.h"
// #include "src/gui.h"

int main(int argc, char *argv[]) {
    ProtonClass ProtonObject;
    ProtonObject.steam = "STEAM_COMPAT_DATA_PATH";
    std::string version = "1.1.2";
    std::cout << "Proton Caller by Avery Murray version: " << version << "\n";
    std::string help_msg = "Usage: [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n 'proton-call 5 ./foo.exe'\n 'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n";

    // check for compat data path
    if (getenv(ProtonObject.steam) == nullptr) {
        std::cout << "Please add " << ProtonObject.steam << " to your environment:\n export" << ProtonObject.steam << "=$HOME/proton/\n" << help_msg;
        exit(EXIT_FAILURE);
    }

    std::cout << ProtonObject.steam << " located at: " << getenv(ProtonObject.steam) << "\n";
    if (argc == 1){std::cout << help_msg;exit(EXIT_FAILURE);}
    if (argv[1] != nullptr){ProtonObject._argv1 = argv[1];}else{exit(EXIT_FAILURE);}
    if (argv[2] != nullptr){ProtonObject._argv2 = argv[2];}else{exit(EXIT_FAILURE);}

    if (ProtonObject._argv1 == "-h"){std::cout << help_msg;exit(EXIT_SUCCESS);}
    if (ProtonObject._argv1 == "-c") {
        ProtonObject.custom = true;
        if (argv[3] != nullptr){
            ProtonObject._argv3 = argv[3];
        } else{exit(EXIT_FAILURE);}
    } else{ProtonObject.custom = false;}

    ProtonObject.setup();
    ProtonObject.check();

    ProtonObject.proton_call();

    return 0;
}
