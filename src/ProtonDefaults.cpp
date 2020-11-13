//
// Created by avery on 07/11/2020.
//

#include "ProtonCaller.h"
#include "ProtonDefaults.h"


bool commonDefault() {
    std::string _home_ = getenv("HOME");
    std::string def = _home_ + "/.steam/steam/steamapps/common/";
    char common[200];
    if (std::filesystem::is_directory(common)) {
        return true;
    }
    return false;
}
