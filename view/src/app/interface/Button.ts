import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import { Functionality } from "../functionality/functionality";
import {
    faArrowPointer,
    faPenNib,
    faBezierCurve,
    faPaintBrush,
} from "@fortawesome/free-solid-svg-icons";
import { MoveCoordService } from "../functionality/move-coord.service";
import { inject } from "@angular/core";
import { AddRemoveCoordService } from "../functionality/add-remove-coord.service";
import { ToggleHandleService } from "../functionality/toggle-handle.service";
import { DrawShapeService } from "../functionality/draw-shape.service";

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
        { title: "Edit curve", icon: faBezierCurve, functionality: inject(ToggleHandleService) },
        { title: "Draw shape", icon: faPaintBrush, functionality: inject(DrawShapeService) },
    ];
}
