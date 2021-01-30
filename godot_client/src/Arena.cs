using Godot;
using System;

using CardGame;

public class Arena : ArenaFs
{
    [Signal]
    public delegate void BattleEnded();
}

