import React, {useMemo} from 'react';
import {Button, Col, Divider, Grid, Loader, Stack, Title} from "@mantine/core";
import {useNavigate} from "react-router-dom";
import useDocuments from "../hooks/useDocuments";
import DocumentGrid from "../components/molecules/DocumentGrid";

export default function Home() {
    const navigate = useNavigate();
    const { isLoading, data } = useDocuments();

    return <Stack />
}
