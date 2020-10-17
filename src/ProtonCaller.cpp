//
// Created by avery on 15/10/2020.
//
#include "ProtonCaller.h"

void Args(ProtonClass &ProtonObject, int argc, char *argv[]) {
    std::string help_msg = "Usage: [-c, -h][5.13, 5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n 'proton-call 5 ./foo.exe'\n 'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n";
    if (argc == 1){std::cout << help_msg;exit(EXIT_FAILURE);}
    if (argv[1] != nullptr){ProtonObject._argv1 = argv[1];}
    else{exit(EXIT_FAILURE);}
    if (ProtonObject._argv1 == "-h"){std::cout << help_msg;exit(EXIT_SUCCESS);}

    if (ProtonObject._argv1 == "-c") {
        ProtonObject.custom = true;
        if (argv[3] != nullptr){
            ProtonObject._argv3 = argv[3];
        } else{exit(EXIT_FAILURE);}
    } else{ProtonObject.custom = false;}

    if (argv[2] != nullptr){ProtonObject._argv2 = argv[2];}
    else{std::cout<<"What program?\n";exit(EXIT_FAILURE);}
}
