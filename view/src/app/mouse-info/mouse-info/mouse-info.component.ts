import { Component } from "@angular/core";
import { MouseInfoService } from "../mouse-info.service";
import { map } from "rxjs/operators";
import { Observable } from "rxjs";
import { CameraService } from "src/app/functionality/camera.service";

@Component({
    selector: "app-mouse-info",
    templateUrl: "./mouse-info.component.html",
    styleUrls: ["./mouse-info.component.scss"],
})
export class MouseInfoComponent {
    protected x$: Observable<string> = this.mouseInfo.normalizedMousePos$.pipe(
        map((coords) => coords.x.toFixed(3)),
    );
    protected y$: Observable<string> = this.mouseInfo.normalizedMousePos$.pipe(
        map((coords) => coords.y.toFixed(3)),
    );
    protected zoom$: Observable<string> = this.cameraService.zoom$.pipe(
        map((zoom) => (zoom * 100).toFixed(0) + "%"),
    );

    constructor(
        private mouseInfo: MouseInfoService,
        private cameraService: CameraService,
    ) {}
}
