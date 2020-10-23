#include <iostream>
#include <string>
#include "src/ProtonCaller.h"

int main(int argc, char *argv[]) {
    ProtonClass ProtonObject;
    ProtonObject.steam = "STEAM_COMPAT_DATA_PATH";
    ProtonObject.common = "PC_COMMON";
    std::string version = "1.2.0";
    std::cout << "Proton Caller by Avery Murray version: " << version << "\n";

    // check for compat data path
    if (getenv(ProtonObject.steam) == nullptr) {
        std::cout << "Please add " << ProtonObject.steam << " to your environment:\n export" << ProtonObject.steam << "=$HOME/proton/\n";
        exit(EXIT_FAILURE);
    }

    std::cout << ProtonObject.steam << " located at: " << getenv(ProtonObject.steam) << "\n";
    Args(ProtonObject, argc, argv);
    setEnvironment(ProtonObject);
    ProtonObject.check();
    ProtonObject.protonCall();
    return 0;
}
