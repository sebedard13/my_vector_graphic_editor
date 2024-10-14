import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from "@angular/core";
import { FormsModule } from "@angular/forms";

@Component({
    selector: "app-number-input",
    templateUrl: "./number-input.component.html",
    styleUrl: "./number-input.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: true,
    imports: [FormsModule],
})
export class NumberInputComponent {
    @Input() id!: string;

    @Input() set value(value: number) {
        if (this.numberValue === value || value === undefined) {
            return;
        }
        this.numberValue = value;
        this.inputValue = this.numberToString(this.numberValue);
    }

    @Input() precisionShown = 3;

    @Output() valueChange = new EventEmitter<number>();

    protected inputValue!: string;
    private numberValue!: number;

    // prettier-ignore
    private numberAndPointAndMath = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ".", "(", ")", "/", "*", "+", "-", " ",
    ];

    protected onValueChange(): void {
        try {
            this.numberValue = this.doMath(this.inputValue);
        } catch (e) {
            // User has type invalid math
        }

        this.inputValue = this.numberToString(this.numberValue);

        this.valueChange.emit(this.numberValue);
    }

    private numberToString(value: number): string {
        let toString = value.toString();
        if (
            toString.includes(".") &&
            toString.length - toString.indexOf(".") > this.precisionShown
        ) {
            toString = value.toFixed(this.precisionShown);
        }
        return toString;
    }

    private doMath(operation: string): number {
        const validForMath = containsUndesiredCharacters(operation, this.numberAndPointAndMath);

        if (!validForMath) {
            throw new Error("Invalid characters in operation string");
        }

        const value = parse(operation);

        if (isNaN(value)) {
            throw new Error("Could not parse string : " + operation);
        }

        return value;
    }
}

function parse(str: string): number {
    try {
        return Function(`'use strict'; return (${str})`)();
    } catch (e) {
        throw new Error("Could not parse string : " + str);
    }
}

function containsUndesiredCharacters(value: string, validChar: string[]): boolean {
    for (const char of value) {
        if (!validChar.includes(char)) {
            return false;
        }
    }
    return true;
}
