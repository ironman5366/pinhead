import React from "react"
import {Grid} from "@mantine/core";
import DocumentCard from "../atoms/DocumentCard";
import PinheadDocument from "../../models/pinheadDocument";

export interface DocumentGridProps {
    documents: PinheadDocument[]
}

export default function DocumentGrid({ documents }: DocumentGridProps) {
    console.log("Documents", documents)
    return <Grid>
        {documents.map((d) =>
            <Grid.Col md={3} key={d.id}>
                <DocumentCard document={d} />
            </Grid.Col>
        )}
    </Grid>
}
