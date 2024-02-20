import { Injectable } from "@angular/core";
import { ScenesService } from "./scenes.service";
import { render_cover } from "wasm-vgc";

@Injectable({
    providedIn: "root",
})
export class RenderService {
    constructor(private scenesServices: ScenesService) {}

    public render() {
        this.scenesServices.currentSceneNow((scene) => {
            const canvas = document.createElement("canvas");
            const ctx = canvas.getContext("2d")!;

            const rect = scene.canvasContent.get_render_rect();
            canvas.width = rect.width();
            canvas.height = rect.height();

            render_cover(ctx, scene.canvasContent, rect.width(), rect.height());
            rect.free();

            const a = document.createElement("a");
            a.href = canvas.toDataURL("image/png");
            a.download = scene.metadata.name + ".png";
            a.click();
        });
    }
}
