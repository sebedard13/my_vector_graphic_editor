import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import { Functionality } from "../functionality/Functionality";
import { faArrowPointer, faPenNib, faBezierCurve } from "@fortawesome/free-solid-svg-icons";
import { MoveCoordService } from "../functionality/move-coord";

export interface Button {
    title: string;
    icon: IconDefinition;
    functionality?: Functionality;
}

export const toolsbarSvgBtn: Button[] = [
    { title: "Select and move", icon: faArrowPointer, functionality: new MoveCoordService() },
    { title: "Add coord to shape or new shape", icon: faPenNib },
    { title: "Edit curve", icon: faBezierCurve },
];
