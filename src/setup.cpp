//
// Created by avery on 20/10/2020.
//

#include "setup.h"
#include "ProtonCaller.h"

void findCommon(ProtonClass &ProtonObject) {

    if (getenv(ProtonObject.common) != nullptr) {
        std::cout << ProtonObject.common << " located at: ";
        ProtonObject.common = getenv(ProtonObject.common);
        std::cout << ProtonObject.common << std::endl;
    } else {
        // defineCommon();
        std::cout << "\nPlease add a 'PC_COMMON' variable to your environment variables which point at the 'steamapps/common/' where your proton versions are installed.\n\n"
                     "export PC_COMMON='/steam/location/common/'\n";
        exit (EXIT_FAILURE);
    }

}

// char defineCommon(){}
