import React, { useState, useEffect } from "react";
import Layout from "../Layout";
import Topbar from "../components/Topbar";
import { useParams } from "react-router-dom";
import MonacoEditor from 'react-monaco-editor';
import fetchEditorThemes from '../EditorThemes';

import { AES, enc } from 'crypto-js';

export default function PastePage() {
    let themeList = fetchEditorThemes();
    
    let { pasteId } = useParams();

    let [ paste, setPaste ] = useState({});
    let [ editorLanguage, setEditorLanguage ] = useState("rust");
    let [ editorTheme, setEditorTheme ] = useState("vs-dark");
    let [ editor, setEditor ] = useState({});

    useEffect(() => {
        fetch(`${window.location.origin}/api/paste/${pasteId}`)
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
                setEditorLanguage(response.paste.language);
            })
            .catch(error => {
                console.log(error);

                setPaste({ data: "Failed to retrieve paste, or paste does not exist." })
            });
    }, [pasteId])

    function onEditorMount(editor, monaco) {
        themeList.forEach(value => {
            monaco.editor.defineTheme(value.id, value.theme);
        })

        setEditor(editor);
    }

    function decryptContents(secret) {
        editor.setValue(AES.decrypt(paste.data, secret).toString(enc.Utf8));
    }

    return (
        <Layout>
            <Topbar 
                paste={paste}
                onThemeChange={(value) => setEditorTheme(value)}
                onLanguageChange={(value) => setEditorLanguage(value)}
                defaultThemeValue="vs-dark"
                defaultLanguage="rust"
                onDecrypt={decryptContents}
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
                editorDidMount={onEditorMount}
                disabled
            />
        </Layout>
    );
}
