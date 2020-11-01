//
// Created by avery on 28/10/2020.
//

#include "ProtonCaller.h"
#include "ProtonArguments.h"
#include "ProtonSetup.h"

void setup(const char *reason) {
    message();
    std::cout << "Starting setup because: " << reason << ".\n\n";
    std::cout << "export " << STEAM << "=$HOME/proton\n"
                                       "mkdir ~/proton\n";
    std::cout << "export " << COMMON << "=y(path to steam's common folder).\n\n";
    std::cout << "Please add these to your environment and install Proton to your " << COMMON << " directory.\n"
                                                                                                 "Proton and Proton-Caller will not work without these.\n";
    std::string input;
    std::cout << "Continue? [y/n]: ";
    std::cin >> input;
    if (input == "y") {
        // system("clear");
        protonMenu();
    }
    exit(EXIT_SUCCESS);
}

void protonMenu() {
    std::cout << "\n"
                 " ------------------------------Menu------------------------------\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |  1. Create Proton Folder and environment variable.           |\n"
                 " |                                                              |\n"
                 " |  2. Create PC_COMMON environment variable.                   |\n"
                 " |                                                              |\n"
                 " |  3. All of the above.                                        |\n"
                 " |                                                              |\n"
                 " |  4. Exit.                                                    |\n"
                 " |                                                              |\n"
                 " ----------------------------------------------------------------\n\n";
    char input[1];
    std::cin >> input;
    int inputVal = atoi(input);
    switch (inputVal) {
        case 1:
            mkdir(findProfile());
            break;
        case 4:
            exit(EXIT_SUCCESS);
    }
}

void mkdir(const char *profile) {
    std::string _proton = "/proton";
    std::string dir = getenv("HOME") + _proton;
    std::filesystem::create_directory(dir);
    std::cout << dir << " created.\n";
    
}

const char *findProfile() {
    std::cout << "Where is your shell profile file? (.bashrc, .zshrc, etc…) ";
    const char *profile;
    return profile;
}
