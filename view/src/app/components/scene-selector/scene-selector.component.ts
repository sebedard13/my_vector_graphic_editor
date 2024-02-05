import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ScenesService } from "src/app/scenes.service";
import { faXmark } from "@fortawesome/free-solid-svg-icons";
import { ModalService } from "src/app/modal.service";
import { NewSceneComponent } from "src/app/new-scene/new-scene.component";

@Component({
    selector: "app-scene-selector",
    templateUrl: "./scene-selector.component.html",
    styleUrl: "./scene-selector.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SceneSelectorComponent {
    protected faXmark = faXmark;

    constructor(protected scenesService: ScenesService, private modalService: ModalService) {}

    newScene() {
        this.modalService.showModal(NewSceneComponent);
    }
}
