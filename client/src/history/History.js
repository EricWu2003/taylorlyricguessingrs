import React from "react";
import { Box, Table, TableBody, TableCell,
  TableContainer, TableHead, TableRow, Paper
} from "@mui/material";
import QueryMenuBar from "./QueryMenuBar";
import GameTableRow from "./GameTableRow";

export default function HistoryPage() {
  const [games, setGames] = React.useState([]);

  return (
    <Box m={2} display="flex" flexDirection="column" alignItems="center">
      <QueryMenuBar setGames={setGames} />

      <Box width="50%">
        <TableContainer component={Paper}>
          <Table sx={{ minWidth: 650 }} aria-label="simple table">
            <TableHead sx={{background:"#B9D9EB"}}>
              <TableRow>
                <TableCell><strong>Time</strong></TableCell>
                <TableCell><strong>Player</strong></TableCell>
                <TableCell align="right"><strong>Score</strong></TableCell>
                <TableCell align="right"><strong>Details</strong></TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {games.map((game, index) => (
                <GameTableRow game={game} index={index} key={game.uuid} />
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </Box>

    </Box>
  );
}
