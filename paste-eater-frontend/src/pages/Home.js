import React, { useState, useEffect } from "react";
import Layout from "../Layout";
import Topbar from "../components/Topbar";
import MonacoEditor from 'react-monaco-editor';
import fetchEditorThemes from '../EditorThemes';
import metadata from 'monaco-editor/esm/metadata';

export default function HomePage() {
    let themeList = fetchEditorThemes();

    let [ editorLanguage, setEditorLanguage ] = useState("rust");
    let [ editorTheme, setEditorTheme ] = useState("vs-dark");

    function beforeEditorMount(editor, monaco) {
        themeList.forEach(value => {
            monaco.editor.defineTheme(value.id, value.theme);
        })
    }

    console.log(metadata.languages);

    return (
        <Layout>
            <Topbar 
                onThemeChange={(value) => setEditorTheme(value)}
                onLanguageChange={(value) => setEditorLanguage(value)}
                defaultThemeValue="vs-dark"
            />
            <MonacoEditor
                language="javascript"
                theme={editorTheme}
                height="95vh"
                options={{
                    selectOnLineNumbers: true
                }}
                editorDidMount={beforeEditorMount}
            />
        </Layout>
    );
}
