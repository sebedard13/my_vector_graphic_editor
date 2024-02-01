import { Injectable } from "@angular/core";
import {
    BehaviorSubject,
    EMPTY,
    Observable,
    combineLatest,
    empty,
    mergeMap,
    of,
    take,
    throwError,
} from "rxjs";
import { CanvasContent, load_from_arraybuffer, save_to_arraybuffer } from "wasm-vgc";

@Injectable({
    providedIn: "root",
})
export class ScenesService {
    private indexCurrentSceneSubject = new BehaviorSubject<number | null>(null);
    public currentScene$: Observable<CanvasContent>;

    private scenesSubject = new BehaviorSubject<CanvasContent[]>([]);
    public scenes$: Observable<CanvasContent[]> = this.scenesSubject.asObservable();

    public scenesList$: Observable<{ canvas: CanvasContent; isCurrent: boolean }[]>;

    constructor() {
        this.scenesSubject.next([CanvasContent.default_call()]);
        this.currentScene$ = combineLatest([this.scenes$, this.indexCurrentSceneSubject]).pipe(
            mergeMap(([scenes, index]) => {
                if (index === null) {
                    return EMPTY;
                }
                return of(scenes[index]);
            }),
        );
        this.indexCurrentSceneSubject.next(0);

        this.scenesList$ = combineLatest([this.scenes$, this.indexCurrentSceneSubject]).pipe(
            mergeMap(([scenes, index]) => {
                if (index === null) {
                    //return throwError(() => new Error("No valid current scene, end the pipe"));
                    return EMPTY; // return empty array instead of error
                }

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

        if (index === this.indexCurrentSceneSubject.getValue()) {
            let newIndex: number | null = index - 1;
            if (newIndex < 0) {
                newIndex = null;
            }
            this.indexCurrentSceneSubject.next(newIndex);
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
        this.currentScene$.pipe(take(1)).subscribe((canvasContent) => {
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
}
