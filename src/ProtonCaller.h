//
// Created by avery on 27/10/2020.
//

/*
 *  Proton Caller. Run any Windows program through Proton.
    Copyright (C) 2020  Avery Murray

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

#ifndef PROTON_CALLER_PROTONCALLER_H
#define PROTON_CALLER_PROTONCALLER_H

#include <iostream>
#include <string>
#include <cstring>
#include <filesystem>
#include <unistd.h>
// For the future !
//#include <ncurses.h>
//#include <menu.h>

#define PROGRAM "Proton Caller"
#define AUTHOR "Avery Murray"
#define VERSION "1.3.0"
#define STEAM "STEAM_COMPAT_DATA_PATH"




class ProtonCaller {
public:
    std::string proton;
    std::string program;
    std::string proton_path;
    const char *common;
    bool custom{};

    void check() const {
        if (custom){return;} // just return if in custom mode,
        namespace fs = std::filesystem;
        std::string _proton = "Proton ";
        std::string path = common + _proton + proton + "/";
        char chkPath[50];
        strcpy(chkPath, path.c_str());
        if (fs::is_directory(chkPath)) {
            std::cout << proton << " exists.\n";
        } else {
            std::cout << proton << " does not exist.\n";
            exit(EXIT_FAILURE);
        }
    }

    void protonCall() const {
        std::string exec_ = proton_path + proton + "/proton";
        char cmd[100], win[50];
        strcpy(win, program.c_str());
        strcpy(cmd, exec_.c_str());
        execl(cmd, "proton", "run", win, NULL);
    }
};

void message();

#endif //PROTON_CALLER_PROTONCALLER_H
