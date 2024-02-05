import { Injectable } from "@angular/core";
import { BehaviorSubject, Observable, combineLatest, map, mergeMap, of } from "rxjs";
import { environment } from "src/environments/environment";
import { CanvasContent, load_from_arraybuffer, save_to_arraybuffer } from "wasm-vgc";

@Injectable({
    providedIn: "root",
})
export class ScenesService {
    private indexCurrentSceneSubject = new BehaviorSubject<number | null>(null);
    private scenesSubject = new BehaviorSubject<CanvasContent[]>([]);

    public scenes$: Observable<CanvasContent[]> = this.scenesSubject.asObservable();

    public currentSceneChange$: Observable<void> = this.indexCurrentSceneSubject.pipe(
        map(() => {}),
    );

    public hasScenes$: Observable<boolean> = this.scenes$.pipe(map((scenes) => scenes.length > 0));
    public scenesList$: Observable<{ canvas: CanvasContent; isCurrent: boolean }[]>;

    constructor() {
        if (environment.openWithTestScenes) {
            this.scenesSubject.next([CanvasContent.default_call()]);
            this.indexCurrentSceneSubject.next(0);
        }

        this.scenesList$ = combineLatest([this.scenes$, this.indexCurrentSceneSubject]).pipe(
            mergeMap(([scenes, index]) => {
                return of(
                    scenes.map((canvas, i) => {
                        return {
                            canvas,
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
                    const canvasContent = load_from_arraybuffer(new Uint8Array(buffer));
                    canvasContent.set_name(file.name);

                    const scenes = this.scenesSubject.getValue();
                    scenes.push(canvasContent);
                    this.scenesSubject.next(scenes);
                    this.indexCurrentSceneSubject.next(scenes.length - 1);
                };
                reader.readAsArrayBuffer(file);
            }
        };
        input.click();
    }

    public saveSceneToFile(): void {
        this.currentSceneNow((canvasContent) => {
            const array = save_to_arraybuffer(canvasContent);
            const url = URL.createObjectURL(new Blob([array]));

            const a = document.createElement("a");
            a.href = url;
            a.download = canvasContent.get_name() + ".vgc";
            a.click();
        });
    }

    public addNewScene(width: number, height: number, name: string) {
        const canvasContent = new CanvasContent(width, height);
        canvasContent.set_name(name);

        const scenes = this.scenesSubject.getValue();
        scenes.push(canvasContent);
        this.scenesSubject.next(scenes);
        this.indexCurrentSceneSubject.next(scenes.length - 1);
    }

    public currentSceneNow(callback: (canvasContent: CanvasContent) => void) {
        const indexScene = this.indexCurrentSceneSubject.getValue();
        const scenes = this.scenesSubject.getValue();
        if (indexScene === null || scenes.length === 0) {
            return;
        }

        return callback(scenes[indexScene]);
    }

    public currentScene(): CanvasContent | null {
        const indexScene = this.indexCurrentSceneSubject.getValue();
        const scenes = this.scenesSubject.getValue();
        if (indexScene === null || scenes.length === 0) {
            return null;
        }

        return scenes[indexScene];
    }
}
