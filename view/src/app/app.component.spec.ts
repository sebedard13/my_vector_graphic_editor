import { ComponentFixture, TestBed } from "@angular/core/testing";
import { beforeEach, describe, expect, test } from "vitest";

import { AppComponent } from "./app.component";
import { AppModule } from "./app.module";

describe("AppComponent", () => {
    let component: AppComponent;
    let fixture: ComponentFixture<AppComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [AppComponent],
            imports: [AppModule],
        }).compileComponents();

        fixture = TestBed.createComponent(AppComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    test("should create", () => {
        expect(component).toBeTruthy();
    });
});
