//
// Created by avery on 27/10/2020.
//

#include "ProtonCaller.h"
#include "ProtonArguments.h"
#include "setup.h"

int main(int argc, char *argv[]) {
    // message();
    ProtonCaller proObj;
    Args(proObj, argc, argv);
    setEnvironment(proObj);
    proObj.check();
    proObj.protonCall();
    return 0;
}

void message() {
    std::cout << PROGRAM << " " << VERSION <<
                            "    Copyright (C) 2020  " << AUTHOR << "\n"
                            "    This program comes with ABSOLUTELY NO WARRANTY.\n"
                            "    This is free software, and you are welcome to redistribute it\n"
                            "    under certain conditions.\n\n";
}