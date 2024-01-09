import { Injectable } from "@angular/core";
import { Observable, ReplaySubject } from "rxjs";
import { CanvasContent, load_from_arraybuffer, save_to_arraybuffer } from "wasm-vgc";

@Injectable({
    providedIn: "root",
})
export class ScenesService {
    private indexScene: number | null = null;
    private scenes: CanvasContent[] = [];

    private currentSceneSubject = new ReplaySubject<CanvasContent>(1);
    public currentScene$: Observable<CanvasContent> = this.currentSceneSubject.asObservable();

    constructor() {
        this.scenes.push(new CanvasContent());
        this.indexScene = 0;
        this.currentSceneSubject.next(this.scenes[this.indexScene]);
    }

    public loadSceneFromFile(): void {
        const input = document.createElement("input");
        input.type = "file";
        input.accept = ".vgc";
        input.onchange = () => {
            const files = input.files;
            if (files && files.length > 0) {
                const file = files[0];
                const reader = new FileReader();
                reader.onload = () => {
                    const buffer = reader.result as ArrayBuffer;
                    const canvasContent = load_from_arraybuffer(new Uint8Array(buffer));

                    this.scenes[0] = canvasContent;
                    this.indexScene = this.scenes.length - 1;
                    this.currentSceneSubject.next(this.scenes[this.indexScene]);
                };
                reader.readAsArrayBuffer(file);
            }
        };
        input.click();
    }

    public saveSceneToFile(): void {
        if (this.indexScene === null) {
            return;
        }

        const canvasContent = this.scenes[this.indexScene];

        const array = save_to_arraybuffer(canvasContent);
        const url = URL.createObjectURL(new Blob([array]));

        const a = document.createElement("a");
        a.href = url;
        a.download = "scene.vgc";
        a.click();
    }
}
