#include <iostream>
#include <string>
#include "ProtonCaller.h"

int main(int argc, char *argv[]) {
    ProtonClass ProtonObject;
    ProtonObject.common = "PC_COMMON";
    std::cout << "Proton Caller by Avery Murray version: " << VERSION << ".\n";

    // check for compat data path
    if (getenv(STEAM) != nullptr) {
        std::cout<< STEAM << " located at: " << getenv(STEAM) << "\n";
    } else {
        std::cout << "Please add " << STEAM << " to your environment:\n "
                     "export" << STEAM << "=$HOME/proton/\n";
        exit(EXIT_FAILURE);
    }

    Args(ProtonObject, argc, argv);
    setEnvironment(ProtonObject);
    ProtonObject.check();
    ProtonObject.protonCall();
    return 0;
}
