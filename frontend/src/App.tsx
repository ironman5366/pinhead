import React from 'react';
import {Container, Title} from "@mantine/core";
import Editor from "./components/organisms/Editor";

function App() {
  return <Container fluid>
    <Title>Pinhead</Title>
    <Editor />
  </Container>
}

export default App;
