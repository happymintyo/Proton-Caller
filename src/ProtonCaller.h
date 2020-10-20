//
// Created by avery on 15/10/2020.
//

#ifndef PROTON_CALLER_PROTONCALLER_H
#define PROTON_CALLER_PROTONCALLER_H
#include <iostream>
#include <string>
#include <cstring>
#include <filesystem>

class ProtonClass {
public:
    std::string _argv1;
    std::string _argv2;
    std::string _argv3;
    const char *steam{};
    const char *common{};
    std::string proton;
    std::string program;
    std::string proton_path;
    bool custom{};
    std::string UserHome = getenv("HOME");

    void check() const {
        if (custom){std::cout<<"custom mode does not check for Proton folder.\n";return;}
        namespace fs = std::filesystem;
        std::string prtn = "Proton ";
        std::string tmpPath = common + prtn + proton + "/";
        char chkPath[50];
        strcpy(chkPath, tmpPath.c_str());
        std::cout<<chkPath<<std::endl;
        if(fs::is_directory(chkPath)){std::cout << proton << " exists.\n";return;}
        else {std::cout << proton << " does not exist.\n";exit(EXIT_FAILURE);}
    }

    void protonCall() const {
        std::string exec_ = proton_path + proton + "/proton run " + program;
        char cmd[200];
        strcpy(cmd, exec_.c_str());
        system(cmd);
    }

};

void Args(ProtonClass &ProtonObject, int argc, char *argv[]);

void setEnvironment(ProtonClass &ProtonObject);

#endif //PROTON_CALLER_PROTONCALLER_H
