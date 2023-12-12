import {Button} from "../interface/Button";
import {faBezierCurve, faPenNib, faSlash} from "@fortawesome/free-solid-svg-icons";

export const toolsbarSvgBtn:Button[] = [
    new Button("Move coord", faBezierCurve),
    new Button("Add coord to shape or new shape", faPenNib),
    new Button("Separate or join handle", faSlash),
]