import { Component, Signal, computed } from "@angular/core";
import { MouseInfoService } from "../mouse-info.service";
import { map } from "rxjs/operators";
import { Observable } from "rxjs";
import { CameraService } from "src/app/functionality/camera.service";

type OptionCoordView = { x: string; y: string } | null;

@Component({
    selector: "app-mouse-info",
    templateUrl: "./mouse-info.component.html",
    styleUrls: ["./mouse-info.component.scss"],
})
export class MouseInfoComponent {
    protected zoom$: Observable<string> = this.cameraService.zoom$.pipe(
        map((zoom) => (zoom * 100).toFixed(0) + "%"),
    );

    protected canvasCoord: Signal<OptionCoordView> = computed(() => {
        const mouseCanvasPos = this.mouseInfo.mouseCanvasPosSignal();
        return { x: mouseCanvasPos.x.toFixed(3), y: mouseCanvasPos.y.toFixed(3) };
    });

    protected canvasScreenCoord: Signal<OptionCoordView> = computed(() => {
        const mouseCanvasScreenPos = this.mouseInfo.mouseCanvasScreenPosSignal();
        return { x: mouseCanvasScreenPos.x.toFixed(3), y: mouseCanvasScreenPos.y.toFixed(3) };
    });

    constructor(
        protected mouseInfo: MouseInfoService,
        private cameraService: CameraService,
    ) {}
}
