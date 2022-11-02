import React from "react";
import { Box, Typography } from "@mui/material";


export default function ResultDisplay({guessRes}) {
  if (guessRes.AFM) {
    return (
      <Typography>You're on the right track, but your guess was too short!</Typography>
    );
  }


  const answerComparison = (guess, guessFlags, answer, answerFlags) => {
    return (
      <Box display="flex" flexDirection="row">
        <Box display="flex" flexDirection="column" mr={1} alignItems="flex-end">
          <Typography sx={{ fontFamily: 'Monospace', color: 'gray', fontWeight: 'bold' }}>Yours:</Typography>
          <Typography sx={{ fontFamily: 'Monospace', color: 'gray', fontWeight: 'bold' }}>Actual:</Typography>
        </Box>
        <Box display="flex" flexDirection="column">
          <FlaggedText text={guess} flags={guessFlags}/>
          <FlaggedText text={answer} flags={answerFlags}/>
        </Box>
      </Box>
    )
  }


  if (guessRes.Correct) {
    const {user_guess, answer, points_earned, new_lifeline} = guessRes.Correct;
    return (
      <Box>
        <Typography><span style={{color:'green', fontWeight: 'bold'}}>Correct!</span> You earned <span style={{fontWeight: 'bold'}}>{points_earned}</span> points</Typography>
        {new_lifeline && <Typography>You also got a {new_lifeline} lifeline!</Typography>}
        {answerComparison(user_guess.text, user_guess.flags, answer.text, answer.flags)}
      </Box>
    )
  }

  if (guessRes.Incorrect) {
    const {user_guess, answer} = guessRes.Incorrect;
    return (
      <Box>
        <Typography><span style={{color:'#BA0021', fontWeight: 'bold'}}>Incorrect!</span> The Game is now over. Better luck next time!</Typography>
        {answerComparison(user_guess.text, user_guess.flags, answer.text, answer.flags)}
      </Box>
    )
  }

  if (guessRes.Skipped) {
    const {user_guess, answer} = guessRes.Skipped;
    return (
      <Box>
        <Typography>Skipped question:</Typography>
        {answerComparison(user_guess.text, user_guess.flags, answer.text, answer.flags)}
      </Box>
    )
  }


  return (
    <Typography>Error: Received a guess result in an unexpected format: {JSON.stringify(guessRes)}</Typography>
  );
}

function FlaggedText({text, flags}) {
  const chars = text.split('')

  if (!flags || flags.length === 0) {
    flags = chars.map(c => -1);
  }
  return (
    <Typography sx={{ fontFamily: 'Monospace' }}>
      {chars.map((char, index) => {
        let style = {}
        if (flags[index] === 1) {
          style.color = '#BA0021'
          style.fontWeight = 'bold';
        } else if (flags[index] === 2) {
          style.color = '#BDB76B'
          style.fontWeight = 'bold';
        } else if (flags[index] === -1) {
          style.color = 'darkgray';
          style.fontWeight = 'bold';
        }


        return <span style={style} key={index}>{char}</span>
      })}
    </Typography>
  );
}
