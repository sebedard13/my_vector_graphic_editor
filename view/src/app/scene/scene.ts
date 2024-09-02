import { SceneClient } from "wasm-client";

export class Scene {
    sceneClient: SceneClient;
    metadata: Metadata;

    constructor(sceneClient: SceneClient) {
        this.sceneClient = sceneClient;
        this.metadata = new Metadata();
    }

    free(): void {
        this.sceneClient.free();
    }
}

export class Metadata {
    name: string = "Untitled";
}
