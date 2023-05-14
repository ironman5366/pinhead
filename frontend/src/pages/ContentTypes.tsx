import React from 'react';
import {Grid} from "@mantine/core";
import ContentFieldList from "../components/organisms/ContentFieldList";
import ContentTypeList from "../components/organisms/ContentTypeList";

export default function ContentTypes() {
    return <Grid>
            <Grid.Col span={9} >
                <ContentTypeList />
            </Grid.Col>
            <Grid.Col span={3} >
                <ContentFieldList />
            </Grid.Col>
        </Grid>
}
