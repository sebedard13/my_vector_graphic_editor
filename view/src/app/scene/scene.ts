import { SceneClient } from "wasm-client";

export class Scene {
    canvasContent: SceneClient;
    metadata: Metadata;

    constructor(canvasContent: SceneClient) {
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
