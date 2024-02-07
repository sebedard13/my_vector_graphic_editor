import { Injectable } from "@angular/core";
import { ScenesService } from "./scenes.service";
import { render_full } from "wasm-vgc";

@Injectable({
    providedIn: "root",
})
export class RenderService {
    constructor(private scenesServices: ScenesService) {}

    public render() {
        const width = 500;

        this.scenesServices.currentSceneNow((scene) => {
            const height = width;
            const canvas = document.createElement("canvas");
            const ctx = canvas.getContext("2d")!;
            canvas.width = width;
            canvas.height = height;
            render_full(ctx, scene.canvasContent, width, height);

            const a = document.createElement("a");
            a.href = canvas.toDataURL("image/png");
            a.download = scene.metadata.name + ".png";
            a.click();
        });
    }
}
