import { CanvasContent } from "wasm-vgc";

export class Scene {
    canvasContent: CanvasContent;
    metadata: Metadata;

    constructor(canvasContent: CanvasContent) {
        this.canvasContent = canvasContent;
        this.metadata = new Metadata();
    }

    free(): void {
        this.canvasContent.free();
    }
}

export class Metadata {
    name: string = "Untitled";
}
