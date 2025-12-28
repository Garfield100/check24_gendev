package com.garfield100.gendev.adapter

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.garfield100.gendev.databinding.CarouselListItemBinding
import com.garfield100.gendev.model.Carousel

class CarouselListAdapter(private val carousels: List<Carousel>) :
    RecyclerView.Adapter<CarouselListAdapter.ViewHolder>() {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val binding = CarouselListItemBinding.inflate(
            LayoutInflater.from(parent.context),
            parent,
            false
        )
        return ViewHolder(binding)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder.bind(carousels[position])
    }

    override fun getItemCount() = carousels.size

    inner class ViewHolder(private val binding: CarouselListItemBinding) :
        RecyclerView.ViewHolder(binding.root) {

        fun bind(carousel: Carousel) {
            binding.carouselTitle.text = carousel.title
            binding.carouselRecyclerview.apply {
                layoutManager = LinearLayoutManager(context, LinearLayoutManager.HORIZONTAL, false)
                adapter = CarouselItemAdapter(carousel.items)
            }
        }
    }
}