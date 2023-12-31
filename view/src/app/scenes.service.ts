import { Injectable } from '@angular/core';
import { Observable, ReplaySubject, Subject } from 'rxjs';
import { CanvasContent } from 'wasm-vgc';

@Injectable({
  providedIn: 'root'
})
export class ScenesService {

  private indexScene: number | null = null;
  private scenes: CanvasContent[] = [];

  private currentSceneSubject = new ReplaySubject<CanvasContent>(1);
  public currentScene$: Observable<CanvasContent> = this.currentSceneSubject.asObservable();

  constructor() {
    this.scenes.push(new CanvasContent);
    this.indexScene = 0;
    this.currentSceneSubject.next(this.scenes[this.indexScene]);
  }

}
