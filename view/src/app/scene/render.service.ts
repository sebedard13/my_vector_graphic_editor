import { Injectable } from "@angular/core";
import { ScenesService } from "./scenes.service";
import { ScreenRect } from "../utilities/client/common";

@Injectable({
    providedIn: "root",
})
export class RenderService {
    constructor(private scenesServices: ScenesService) {}

    public render() {
        this.scenesServices.currentSceneNow((scene) => {
            const canvas = document.createElement("canvas");
            const ctx = canvas.getContext("2d")!;

            const rect = new ScreenRect(scene.sceneClient.get_render_rect());
            canvas.width = rect.width();
            canvas.height = rect.height();

            scene.sceneClient.render_cover(ctx, rect.width(), rect.height());

            const a = document.createElement("a");
            a.href = canvas.toDataURL("image/png");
            a.download = scene.metadata.name + ".png";
            a.click();
        });
    }
}
