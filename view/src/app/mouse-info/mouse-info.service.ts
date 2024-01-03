import { Injectable } from "@angular/core"
import { Subject } from "rxjs"

@Injectable({
    providedIn: "root",
})
export class MouseInfoService {
    public normalizedMousePos = new Subject<{ x: number; y: number }>()
    public mousePos = new Subject<{ x: number; y: number }>()
    public zoom = new Subject<number>()

    constructor() {}
}
