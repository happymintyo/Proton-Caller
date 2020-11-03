//
// Created by avery on 28/10/2020.
//

#include "ProtonCaller.h"
#include "ProtonArguments.h"
#include "ProtonSetup.h"
#include <fstream>

void setup(const char *reason) {
    message();
    std::cout << "Starting setup because: " << reason << ".\n\n";
    std::cout << "export " << STEAM << "=$HOME/proton\n"
                                       "mkdir ~/proton\n";
    std::cout << "export " << COMMON << "=(path to steam's common folder).\n\n";
    std::cout << "Please add these to your environment and install Proton to your "
              << COMMON << " directory.\n"
              "Proton and Proton-Caller will not work without these.\n";

    exit(EXIT_SUCCESS);
}

/*
 * Planned
void mkdir() {
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
*/

void message() {
    std::cout << PROGRAM << " " << VERSION <<
              "    Copyright (C) 2020    " << AUTHOR << "\n"
              "    This program comes with ABSOLUTELY NO WARRANTY.\n"
              "    This is free software, and you are welcome to redistribute it\n"
              "    under certain conditions.\n\n";
}

void PRVersion() {
    message();
    std::cout << "\n" <<
              "Compiled at: " << __TIMESTAMP__ << "\n";
}

/* Planned
void createEnv(bool var) {
    std::string shell;
    std::cout << "Where is your shell profile? (~/.bashrc, ~/.zshrc): ";
    std::cin >> shell;
    std::ofstream fProfile;
    fProfile.open(shell,std::ios::out);
    if (var) {
        //fProfile << "\nexport " << STEAM << "=$HOME/proton\n";
    } else {
        std::string cCommon;
        std::cout << "Where is your Steam common directory: ";
        std::cin >> cCommon;
        //system("echo 'export ", COMMON, "=", cCommon);
        //fProfile << "\nexport " << COMMON << "=" << cCommon << "\n";
    }
    fProfile.close();
}
*/
