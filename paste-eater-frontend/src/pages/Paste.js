import React, { useState, useEffect } from "react";
import Layout from "../Layout";
import Topbar from "../components/Topbar";
import { useParams, useNavigate } from "react-router-dom";
import MonacoEditor from 'react-monaco-editor';
import fetchEditorThemes from '../EditorThemes';

export default function PastePage() {
    let themeList = fetchEditorThemes();
    
    let { pasteId } = useParams();

    let [ paste, setPaste ] = useState({});
    let [ editorLanguage, setEditorLanguage ] = useState("rust");
    let [ editorTheme, setEditorTheme ] = useState("vs-dark");

    useEffect(() => {
        fetch(`${process.env.REACT_APP_PASTE_API_LOCATION}/paste/${pasteId}`)
            .then(response => {
                if (response.ok) {
                    return response.json()
                }

                throw response;
            })
            .then(response => {
                if (response.error) {
                    throw response;
                }

                setPaste(response.paste);

                console.log(response);
            })
            .catch(error => {
                console.log(error);

                setPaste({ data: "Failed to retrieve paste, or paste does not exist." })
            });
    }, [])

    function beforeEditorMount(editor, monaco) {
        themeList.forEach(value => {
            monaco.editor.defineTheme(value.id, value.theme);
        })
    }

    return (
        <Layout>
            <Topbar 
                paste={paste}
                onThemeChange={(value) => setEditorTheme(value)}
                onLanguageChange={(value) => setEditorLanguage(value)}
                defaultThemeValue="vs-dark"
            />
            <MonacoEditor
                language={editorLanguage}
                theme={editorTheme}
                value={paste.data}
                height="95vh"
                options={{
                    selectOnLineNumbers: true,
                    readOnly: true
                }}
                editorDidMount={beforeEditorMount}
                disabled
            />
        </Layout>
    );
}
