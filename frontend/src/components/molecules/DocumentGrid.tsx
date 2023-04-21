import React from "react"
import {Grid} from "@mantine/core";
import DocumentCard from "../atoms/DocumentCard";
import PinheadDocument from "../../types/PinheadDocument";

export interface DocumentGridProps {
    documents: PinheadDocument[]
}

export default function DocumentGrid({ documents }: DocumentGridProps) {
    return <Grid>
        {documents.map((d) =>
            <Grid.Col md={3} key={d.id}>
                <DocumentCard document={d} />
            </Grid.Col>
        )}
    </Grid>
}
