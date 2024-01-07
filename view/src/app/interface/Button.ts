import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import { Functionality } from "../functionality/functionality";
import { faArrowPointer, faPenNib, faBezierCurve } from "@fortawesome/free-solid-svg-icons";
import { MoveCoordService } from "../functionality/move-coord.service";
import { inject } from "@angular/core";
import { AddRemoveCoordService } from "../functionality/add-remove-coord.service";

export interface Button {
    title: string;
    icon: IconDefinition;
    functionality?: Functionality;
}

export function toolsbarSvgBtn(): Button[] {
    return [
        { title: "Select and move", icon: faArrowPointer, functionality: inject(MoveCoordService) },
        {
            title: "Add or remove coord",
            icon: faPenNib,
            functionality: inject(AddRemoveCoordService),
        },
        { title: "Edit curve", icon: faBezierCurve },
    ];
}
