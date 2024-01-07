import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import { Functionality } from "../functionality/functionality";
import { faArrowPointer, faPenNib, faBezierCurve } from "@fortawesome/free-solid-svg-icons";
import { MoveCoordService } from "../functionality/move-coord.service";
import { inject } from "@angular/core";

export interface Button {
    title: string;
    icon: IconDefinition;
    functionality?: Functionality;
}

export function toolsbarSvgBtn(): Button[] {
    return [
        { title: "Select and move", icon: faArrowPointer, functionality: inject(MoveCoordService) },
        { title: "Add coord to shape or new shape", icon: faPenNib },
        { title: "Edit curve", icon: faBezierCurve },
    ];
}
