"use client";

import { useState } from "react";
import { SDUIComponentData } from "@/app/sdui";
import { Button } from "@/components/ui/button";
import { SDUIRenderer } from "../../components/sduiComponents/renderer";
import { Textarea } from "@/components/ui/textarea"
import EmptyPage from "../../components/emptyPage";
import exampleJson from "./exampleJson.json"
import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";

export default function Playground() {
    const [widgets, setWidgets] = useState<SDUIComponentData[]>([]);
    const [jsonInput, setJsonInput] = useState(JSON.stringify(exampleJson, null, 2));
    const [error, setError] = useState<string | null>(null);
    const [autoRender, setAutoRender] = useState(true);

    const parseJson = (input: string) => {
        try {
            setError(null);
            if (!input.trim()) return;

            const json = JSON.parse(input);
            const widgets = json instanceof Array ? json : [json]; // accept both arrays and single objects by wrapping the latter

            setWidgets(widgets as SDUIComponentData[]);
        } catch (e) {
            setError("Invalid JSON format");
            console.error(e);
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const newValue = e.target.value;
        setJsonInput(newValue); 

        if (autoRender) {
            parseJson(newValue);
        }
    };

    const handleAutoRenderChange = (checked: boolean) => {
        setAutoRender(checked);
        
        if (checked) {
            parseJson(jsonInput);
        }
    };

    return (
        <EmptyPage>
            <div className="flex flex-col gap-4 mb-8 border-b pb-8">
                <h2 className="text-lg font-semibold">JSON Input</h2>
                <Textarea
                    className="h-48 font-mono text-xs"
                    value={jsonInput}
                    onChange={handleInputChange} 
                />
                <div className="flex items-center gap-4">
                    <Button onClick={() => parseJson(jsonInput)}>Render UI</Button>
                    
                    <div className="flex items-center space-x-2">
                        <Checkbox 
                            id="autoRender" 
                            checked={autoRender}
                            onCheckedChange={(c) => handleAutoRenderChange(c as boolean)}
                        />
                        <Label htmlFor="autoRender" className="cursor-pointer">
                            Auto Render
                        </Label>
                    </div>

                    {error && <p className="text-red-500 text-sm">{error}</p>}
                </div>
            </div>

            {widgets.length > 0 ? (
                <div className="flex flex-col gap-8 w-full">
                    {widgets.map((widget, index) => (
                        <SDUIRenderer key={index} data={widget} />
                    ))}
                </div>
            ) : (
                <p className="text-gray-500">No widgets rendered</p>
            )}
        </EmptyPage>
    );
}