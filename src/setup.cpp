//
// Created by avery on 28/10/2020.
//

#include "ProtonCaller.h"
#include "ProtonArguments.h"
#include "setup.h"

void setup(const char *reason) {
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
        system("clear");
        protonMenu();
    }
    exit(EXIT_SUCCESS);
}

void protonMenu() {
    std::cout << "\n"
                 " ------------------------------Menu------------------------------\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " |                                                              |\n"
                 " ----------------------------------------------------------------\n\n";
}
