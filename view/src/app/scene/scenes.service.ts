import { Injectable } from "@angular/core";
import { BehaviorSubject, Observable, combineLatest, map, mergeMap, of } from "rxjs";
import { environment } from "src/environments/environment";
import { SceneClient } from "wasm-client";
import { Scene } from "./scene";

@Injectable({
    providedIn: "root",
})
export class ScenesService {
    private indexCurrentSceneSubject = new BehaviorSubject<number | null>(null);
    private scenesSubject = new BehaviorSubject<Scene[]>([]);

    public scenes$: Observable<Scene[]> = this.scenesSubject.asObservable();

    public currentSceneChange$: Observable<void> = this.indexCurrentSceneSubject.pipe(
        map(() => {}),
    );

    public hasScenes$: Observable<boolean> = this.scenes$.pipe(map((scenes) => scenes.length > 0));
    public scenesList$: Observable<{ scene: Scene; isCurrent: boolean }[]>;

    constructor() {
        if (environment.openWithTestScenes) {
            this.scenesSubject.next([new Scene(SceneClient.default_call())]);
            this.indexCurrentSceneSubject.next(0);
        }

        this.scenesList$ = combineLatest([this.scenes$, this.indexCurrentSceneSubject]).pipe(
            mergeMap(([scenes, index]) => {
                return of(
                    scenes.map((scene, i) => {
                        return {
                            scene: scene,
                            isCurrent: i === index,
                        };
                    }),
                );
            }),
        );
    }

    public setCurrentScene(index: number): void {
        if (index < 0) {
            return;
        }
        if (index >= this.scenesSubject.getValue().length) {
            return;
        }
        if (index === this.indexCurrentSceneSubject.getValue()) {
            return;
        }
        this.indexCurrentSceneSubject.next(index);
    }

    public removeScene(index: number) {
        const scenes = this.scenesSubject.getValue();
        const deleted = scenes.splice(index, 1);
        this.scenesSubject.next(scenes);
        deleted[0].free();

        const current = this.indexCurrentSceneSubject.getValue();
        if (current === null) {
            return;
        }

        if (index <= current) {
            const newIndex = current - 1;
            if (newIndex < 0) {
                this.indexCurrentSceneSubject.next(null);
            } else {
                this.indexCurrentSceneSubject.next(newIndex);
            }
        }
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
                    const canvasContent = SceneClient.load(new Uint8Array(buffer));
                    const scene = new Scene(canvasContent);
                    let filename = file.name;
                    if (filename.endsWith(".vgc")) {
                        filename = filename.slice(0, -4);
                    }

                    scene.metadata.name = filename;

                    const scenes = this.scenesSubject.getValue();
                    scenes.push(scene);
                    this.scenesSubject.next(scenes);
                    this.indexCurrentSceneSubject.next(scenes.length - 1);
                };
                reader.readAsArrayBuffer(file);
            }
        };
        input.click();
    }

    public saveSceneToFile(): void {
        this.currentSceneNow((scene) => {
            const array = scene.canvasContent.save();
            const url = URL.createObjectURL(new Blob([array]));

            const a = document.createElement("a");
            a.href = url;
            a.download = scene.metadata.name + ".vgc";
            a.click();
        });
    }

    public addNewScene(width: number, height: number, name: string) {
        const canvasContent = new SceneClient(width, height);
        const scene = new Scene(canvasContent);
        scene.metadata.name = name;

        const scenes = this.scenesSubject.getValue();
        scenes.push(scene);
        this.scenesSubject.next(scenes);
        this.indexCurrentSceneSubject.next(scenes.length - 1);
    }

    public currentSceneNow(callback: (scene: Scene) => void) {
        const scene = this.currentScene();
        if (!scene) {
            return;
        }

        return callback(scene);
    }

    public currentScene(): Scene | null {
        const indexScene = this.indexCurrentSceneSubject.getValue();
        const scenes = this.scenesSubject.getValue();
        if (indexScene === null || scenes.length === 0) {
            return null;
        }

        return scenes[indexScene];
    }
}
