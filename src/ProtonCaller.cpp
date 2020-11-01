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

/* Most includes and macros defined in
 * ProtonCaller.h. Look there for everything.
 */
#include "ProtonCaller.h"
#include "ProtonArguments.h"

int main(int argc, char *argv[]) {
    // message();
    ProtonCaller proObj;
    Args(proObj, argc, argv);
    setEnvironment(proObj);
    proObj.check();
    proObj.protonCall();
    return 0;
}
