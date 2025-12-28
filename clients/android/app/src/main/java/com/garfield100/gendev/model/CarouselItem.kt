package com.garfield100.gendev.model

import androidx.annotation.DrawableRes

data class CarouselItem(
    val title: String,
    val description: String,
    @DrawableRes val imageRes: Int? = null
)
