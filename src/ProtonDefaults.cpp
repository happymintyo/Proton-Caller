//
// Created by avery on 07/11/2020.
//

#include "ProtonCaller.h"
#include "ProtonDefaults.h"


bool commonDefault() {
    std::string _home_ = getenv("HOME");
    std::string def = _home_ + "/.steam/steam/steamapps/common/";
    char commonquest[200];
    if (std::filesystem::is_directory(commonquest)) {
        return true;
    } else {
        return false;
    }
}
